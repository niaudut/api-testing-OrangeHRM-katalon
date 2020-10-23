<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Projects</name>
   <tag></tag>
   <elementGuidId>77f09b34-fa7d-44d7-8a7b-48cbf37afe6d</elementGuidId>
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
   <restUrl>${url}/api/v1/project</restUrl>
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
      <id>f8ca3a86-3005-4674-8714-7172f3af05d0</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>66aedb0d-7cca-4edb-b34e-2f3d17919911</id>
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

WS.verifyElementPropertyValue(response, 'data[0].projectId', &quot;2&quot;)
WS.verifyElementPropertyValue(response, 'data[0].projectName', &quot;Klan Uchiha&quot;)
WS.verifyElementPropertyValue(response, 'data[0].customerId', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].customerName', &quot;Sasuke Uchiha&quot;)
WS.verifyElementPropertyValue(response, 'data[0].isDeleted', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].admins.employeeId', &quot;3&quot;)
WS.verifyElementPropertyValue(response, 'data[0].admins.name', &quot;Nia Middle Test&quot;)
WS.verifyElementPropertyValue(response, 'data[0].activities[0].id', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].activities[0].name', &quot;Requirement ke-1&quot;)
WS.verifyElementPropertyValue(response, 'data[0].activities[0].isDeleted', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[0].activities[1].id', &quot;3&quot;)
WS.verifyElementPropertyValue(response, 'data[0].activities[1].name', &quot;test&quot;)
WS.verifyElementPropertyValue(response, 'data[0].activities[1].isDeleted', &quot;0&quot;)

WS.verifyElementPropertyValue(response, 'data[5].projectId', &quot;9&quot;)
WS.verifyElementPropertyValue(response, 'data[5].projectName', &quot;nestle alokasi&quot;)
WS.verifyElementPropertyValue(response, 'data[5].customerId', &quot;11&quot;)
WS.verifyElementPropertyValue(response, 'data[5].customerName', &quot;PT Nestle&quot;)
WS.verifyElementPropertyValue(response, 'data[5].description', &quot;test 456&quot;)
WS.verifyElementPropertyValue(response, 'data[5].isDeleted', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'data[5].admins.employeeId', &quot;4&quot;)
WS.verifyElementPropertyValue(response, 'data[5].admins.name', &quot;Sherina Melodi Darmawan&quot;)
WS.verifyElementPropertyValue(response, 'data[5].activities[0].id', &quot;9&quot;)
WS.verifyElementPropertyValue(response, 'data[5].activities[0].name', &quot;activity 1&quot;)
WS.verifyElementPropertyValue(response, 'data[5].activities[0].isDeleted', &quot;0&quot;)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
