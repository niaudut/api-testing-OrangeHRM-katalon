<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_KPIS</name>
   <tag></tag>
   <elementGuidId>d262702d-2ea1-44d6-809c-dbb0cc1a9fd2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/api/v1/kpis</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>28712a94-6db1-4f4b-b677-47f7aba6d0cf</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>7ba78690-b19d-4dba-9992-55d4c071ff67</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].id', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].jobTitle.id', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].jobTitle.jobTitleName', &quot;QA Engineer&quot;)
WS.verifyElementPropertyValue(response, 'data[0].jobTitle.jobDescription', &quot;Staff&quot;)
WS.verifyElementPropertyValue(response, 'data[0].jobTitle.isDeleted', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].kpi', &quot;25&quot;)
WS.verifyElementPropertyValue(response, 'data[0].minRating', &quot;20&quot;)
WS.verifyElementPropertyValue(response, 'data[0].maxRating', &quot;30&quot;)


WS.verifyElementPropertyValue(response, 'data[1].id', &quot;2&quot;)
WS.verifyElementPropertyValue(response, 'data[1].jobTitle.id', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data[1].jobTitle.jobTitleName', &quot;QA Engineer&quot;)
WS.verifyElementPropertyValue(response, 'data[1].jobTitle.jobDescription', &quot;Staff&quot;)
WS.verifyElementPropertyValue(response, 'data[1].jobTitle.isDeleted', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[1].kpi', &quot;35&quot;)
WS.verifyElementPropertyValue(response, 'data[1].minRating', &quot;30&quot;)
WS.verifyElementPropertyValue(response, 'data[1].maxRating', &quot;50&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
